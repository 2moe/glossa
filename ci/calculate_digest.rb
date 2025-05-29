# frozen_string_literal: true

require 'json'
require 'digest/sha2'
require 'digest/blake3'

# The **release** directory is already created during the execution of docker.rb, so we can proceed to enter it directly at this point
Dir.chdir 'release'
digests = {}
Pathname.glob('*.zst').each do |path|
  file_name = path.basename
  sha256 = Digest::SHA256.file(path).hexdigest
  blake3 = Digest::Blake3.file(path).hexdigest

  digests[file_name] = {
    file_size: path.size?,
    digest: {
      sha256: sha256,
      blake3: blake3
    }
  }
end

# mutable String
markdown = String.new "| File   | Size (bytes)  | SHA256 | Blake3 |\n"
markdown <<           "|--------|---------------|--------|--------|\n"

digests.each do |file, info|
  digest = info[:digest]
  sha256 = digest[:sha256]
  blake3 = digest[:blake3]
  size   = info[:file_size]

  markdown << "| #{file} | #{size} | `#{sha256}` | `#{blake3}` |\n"
end

File.write('digests.md', markdown)

JSON.pretty_generate(digests)
    .then { File.write('digests.json', _1) }
