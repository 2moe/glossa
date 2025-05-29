def install_gem(req: 'digest/blake3', gem_name: 'blake3-rb')
  require req
rescue LoadError => e
  p e
  %w[gem install --user-install].push(gem_name).then(&run)
  # File.write(ENV['GITHUB_PATH'], "#{Gem.user_dir}/bin\n", mode: 'a')
end
