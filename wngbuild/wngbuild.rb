class BuildProfile
  def initialize(files, output)
    @files = files
    @output = output
    @cc = "gcc"
    @flags = ""
  * Parameters
  def cc=(cc)
    @cc=cc    
  end
  def cc
    @cc
  end
  def flags=(flags)
    @flags=flags
  end
  * Methods  
  def run
    system(@cc, @files, "-o", @output, @flags)
  end
  def runOutput
    system("./#{@output}")
  end
end
