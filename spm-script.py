# This files is used to create spm package easily

import json
from os import system,getcwd


name = input("Enter the name of the package: ")
version = input("Enter the version of the package: ")
dependencies = input("Enter the dependencies of the package: ")
dependencies = dependencies.split(" ")
prepare_info = input("Enter the the command to prepare the package (leave blanck if none) : ")
make_info = input("Enter the the command to make the package (leave blanck if none) : ")
test_info = input("Enter the the command to test the package (leave blanck if none) : ")
install_info = input("Enter the the command to install the package (use $BUILD_ROOT for the install dir ) : ")
special_info = input("Commands to be executed after the install : ")

sourcePath = input("Enter the path to the source directory : ")

basicPattern = """
{
    "name" : "",
    "type" : "",
    "version" : "",
    "dependencies" : [],
    "info" : 
    {
        "prepare" : "",
        "make" : "",
        "test" : "",
        "install" : "",
        "special" : ""
        
    },
    "locations" : []
}
"""
parsedPattern = json.loads(basicPattern)
print(parsedPattern)
parsedPattern["name"] = name
parsedPattern["type"] = "src"
parsedPattern["version"] = int(version)
for dependencie in dependencies:
    parsedPattern["dependencies"].append(dependencie)
parsedPattern["info"]["prepare"] = prepare_info
parsedPattern["info"]["make"] = make_info
parsedPattern["info"]["test"] = test_info
parsedPattern["info"]["install"] = install_info
parsedPattern["info"]["special"] = special_info

tempDir = ("/tmp/%s.src.spm.tmp" % name)


system(("mkdir %s" % tempDir))
system(("tar -xvf %s -C %s" % (sourcePath, tempDir)))

spm = open(("%s/%s.spm" % (tempDir, name)), "w")
spm.write(json.dumps(parsedPattern))
spm.close()

packageArchivePath = ("%s/%s.src.spm.tar.gz" % (getcwd(),name))

system(("( cd %s && tar -cvf %s * ) " % (tempDir,packageArchivePath )))
system(("rm -rf %s" % tempDir))




