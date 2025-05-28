# ------------------
def run = ->(cmd) { system(*cmd) }

# run_in_background
def run_in_bg = ->(cmd) { Process.spawn(*cmd) }

require 'pathname'
# ------------------
load ARGV[0]
