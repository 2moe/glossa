# ------------------
def run = ->(cmd) do
    p cmd
    system(*cmd)
end

# run_in_background
def run_in_bg = ->(cmd) do
    p cmd
    Process.spawn(*cmd)
end

require 'pathname'
# ------------------
load ARGV[0]
