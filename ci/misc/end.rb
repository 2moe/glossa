# ------------------
def run = ->(cmd) do
  p cmd
  success = system(*cmd)
  raise "Command failed: #{cmd}" unless success
end

# run_in_background
def run_in_bg = ->(cmd) do
  p cmd
  Process.spawn(*cmd)
end

def wait_task(pid = nil)
  if pid
    Process.wait pid
  end

  status = $?.exitstatus
  raise "Command failed with status #{status}" unless status == 0
end

require 'pathname'
# ------------------
load ARGV[0]
