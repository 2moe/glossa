# frozen_string_literal: true

# rubocop:disable Metrics/MethodLength, Lint/MissingCopEnableDirective
# --------------------
# Docker Utilities
# --------------------
require 'fileutils'

DOCKER_CONTEXT_TMP = 'docker_tmp'

# Container registry repository from environment variable
GHCR_REPO = ENV.fetch('ghcr_repo') do
  abort 'WARNING: ghcr_repo environment variable not set'
end.freeze

# Converts a hash to Docker output format string
#
# @example {type: 'image', compression: 'zstd'} => "type=image,compression=zstd"
def convert_to_docker_output(hash)
  hash.map do |key, value|
    "#{key.to_s.tr('_', '-')}=#{value}"
  end.join(',')
end

# Compresses a file using zstd
#
# @param tag [String] Image tag for naming the release
# @param target [String] Build target architecture
# @param pkg_name [String] Cargo package/crate name
# @param suffix [String] File suffix
#
# @return [Integer] PID of the background compression process
def compress_file(tag: 'wasi-p2', target: 'wasm32-wasip2', pkg_name: 'glossa-cli', suffix: '.wasm')
  fs = FileUtils
  tmp = DOCKER_CONTEXT_TMP
  fs.mkdir_p tmp
  fs.mkdir_p 'release'

  src = "target/#{target}/thin/#{pkg_name}#{suffix}"
  dst = "release/#{tag}#{suffix}"
  # Copy source file
  fs.cp(src, tmp)
  fs.cp(src, dst)

  # Run compression in background
  {
    zstd: nil,
    # Use `-T0` for multithreaded compression (more portable than zstdmt)
    "-T0": nil,
    # Delete input file(s) after the operation completes successfully
    rm: true,
    force: true,
    verbose: true,
    # level
    "-19": nil
  }
    .then(&hash_to_args)
    .concat([dst, '-o', "#{dst}.zst"])
    .then(&run_in_bg)
end

# --------------------
# Docker Builder Setup
# --------------------
# Failure of this operation is acceptable; in case of failure, the default buildx-node/buildkit will be used instead.
def create_zstd_buildx_machine
  system 'docker buildx create --use --name zstd' or
    warn 'Failed to use the new buildx-machine'
end

# --------------------
# Docker Build Process
# --------------------
# Common output configuration
BUILDKIT_OUTPUT = {
  type: 'image',
  oci_mediatypes: true,
  compression: 'zstd',
  force_compression: false,
  compression_level: 18,
  attestation_inline: false
}.freeze

# Common Docker build options
DOCKER_BUILD_OPTIONS = {
  docker: nil,
  buildx: nil,
  build: nil,
  platform: 'wasi/wasm',
  output: convert_to_docker_output(BUILDKIT_OUTPUT),
  push: true,
  tag: '',

  # String, not bool.
  # provenance: "false",

  file: 'ci/wasi.dockerfile',
  "#{DOCKER_CONTEXT_TMP}": nil
}.freeze

def build_images(targets)
  targets.each do |config|
    # Start compression in background
    pid = compress_file(**config.slice(:tag, :target))

    # Prepare Docker command
    DOCKER_BUILD_OPTIONS.merge(
      {
        platform: config[:platform],
        tag: "#{GHCR_REPO}:#{config[:tag]}",
      }
    ).then(&run)

    # Wait for compression to complete
    wait_task pid
  end
end

# Builds WASI targets and handles compression
def build_wasi
  # Build target configurations
  [
    {
      # tag: "#{GHCR_REPO}:wasi-p1",
      tag: 'wasi-p1',
      target: 'wasm32-wasip1',
      platform: 'wasip1/wasm'
    },
    {
      tag: 'wasi-p2',
      target: 'wasm32-wasip2',
      platform: 'wasi/wasm'
    }
  ].then { build_images _1 }
end

def create_and_push_manifest(tags)
  %w[
    docker manifest
    create --amend
  ]
    .concat(tags)
    .then(&run)

  %W[docker manifest push --purge #{tags.first}].then(&run)
end

def build_and_push_zstd_docker_image(target: 'wasi', create_manifest: true)
  create_zstd_buildx_machine

  case target
  when 'wasi'
    build_wasi
    tags =
      ['latest'].concat([1, 2].map { |n| "wasi-p#{n}" })
                .map { "#{GHCR_REPO}:#{_1}" }
  end

  return unless create_manifest

  create_and_push_manifest(tags)
end
