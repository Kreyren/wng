import subprocess as sp

class BuildProfile:
    def __init__(self, files, output):
        self.files = files
        self.output = output
        self.cc = "gcc"
        self.flags = ""

    def run(self):
        cmd = f"{self.cc} {self.files} -o {self.output} {self.flags}"
        sp.call(cmd)

    def runOutput(self):
        cmd = f"./{self.output}"
        sp.call(cmd)
