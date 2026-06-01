# Annex J: Annex J

## Annex J
(normative)
Inclusion of foreign language code
J.1 General
This annex describes common guidelines for the inclusion of foreign language code into a SystemVerilog
application. The intention of these guidelines is to enable the redistribution of C binaries in shared object
form.
J.2 Overview
Foreign language code is functionality that is included into SystemVerilog using the DPI. As a result, all
statements of this annex apply only to code included using this interface; code included by using other
interfaces (e.g., VPI) is outside the scope of this standard. Due to the nature of the DPI, most foreign
language code is usually created from C or C++ source code, although nothing precludes the creation of
appropriate object code from other languages. This annex adheres to this rule: its content is independent
from the actual language used.
In general, foreign language code is provided in the form of object code compiled for the actual platform.
The capability to include foreign language code in object-code form shall be supported by all simulators as
specified here.
This annex defines how to
— Specify the location of the corresponding files within the filesystem.
— Specify the files to be loaded (in case of object code).
— Provide the object code (as a shared library or archive).
Although this annex defines guidelines for a common inclusion methodology, it requires multiple
implementations (usually two) of the corresponding facilities. This takes into account that multiple users can
have different viewpoints and different requirements on the inclusion of foreign language code.
— A vendor that wants to provide its intellectual property (IP) in the form of foreign language code
often requires a self-contained method for the integration, which still permits an integration by a
third party. This use case is often covered by a bootstrap file approach.
— A project team that specifies a common, standard set of foreign language code might change the
code depending on technology, selected cells, back-annotation data, and other items. This use case is
often covered by a set of tool switches, although it might also use the bootstrap file approach.
— An user might want to switch between selections or provide additional code. This use case is
covered by providing a set of tool switches to define the corresponding information, although it
might also use the bootstrap file approach.
NOTE—This annex defines a set of switch names to be used for a particular functionality. This is of informative nature;
the actual naming of switches is not part of this standard. Further, it might not be possible to use certain character
configurations in all operating systems or shells. Therefore, any switch name defined within this standard is a
recommendation on how to name a switch, but not a requirement of the language.

<!-- Page 1304 -->

IEEE Std J.3 Location independence
All path names specified within this annex are intended to be location independent, which is accomplished
by using the switch -sv_root. It can receive a single directory path name as the value, which is then
prepended to any relative path name that has been specified. In absence of this switch, or when processing
relative file names before any -sv_root specification, the current working directory of the user shall be
used as the default value.
J.4 Object code inclusion
Compiled object code is required for cases where the compilation and linking of source code are fully
handled by the user; thus, the created object code only need be loaded to integrate the foreign language code
into a SystemVerilog application. All SystemVerilog applications shall support the integration of foreign
language code in object code form. FigureJ.1 depicts the inclusion of object code and its relations to the
various steps involved in this integration process.
Performed by the user Object code
inclusion
Source System-
Object
code Verilog
code
application
Compile Link Load
Figure J.1—Inclusion of object code into a SystemVerilog application
Compiled object code can be specified by one of the following two methods:
a) By an entry in a bootstrap file; see J.4.1 for more details on this file and its content. Its location shall
be specified with one instance of the switch -sv_liblist pathname. This switch can be used
multiple times to define the usage of multiple bootstrap files.
b) By specifying the file with one instance of the switch -sv_lib pathname_without_
extension (i.e., the file name shall be specified without the platform-specific extension). The
SystemVerilog application is responsible for appending the appropriate extension for the actual
platform. This switch can be used multiple times to define multiple libraries holding object code.
Both methods shall be provided and made available concurrently to permit any mixture of their usage. Every
location can be an absolute path name or a relative path name, where the value of the switch -sv_root is
used to identify an appropriate prefix for relative path names (see J.3 for more details on forming path
names).
The following conditions also apply:
— The compiled object code itself shall be provided in the form of a shared library having the
appropriate extension for the actual platform.
NOTE—Shared libraries use, for example, .so for Solaris and .sl for HP-UX; other operating systems might use
different extensions. In any case, the SystemVerilog application needs to identify the appropriate extension.
— The provider of the compiled code is responsible for any external references specified within these
objects. Appropriate data need to be provided to resolve all open dependencies with the correct
information.

<!-- Page 1305 -->

IEEE Std — The provider of the compiled code shall avoid interferences with other software and select the
appropriate software version (e.g., in cases where two versions of the same library are referenced).
Similar problems can arise when there are dependencies on the expected run-time environment in
the compiled object code (e.g., in cases where C++ global objects or static initializers are used).
— The SystemVerilog application need only load object code within a shared library that is referenced
by the SystemVerilog code or by registration functions; loading of additional functions included
within a shared library can interfere with other parts.
In the case of multiple occurrences of the same file (files having the same path name or that can easily be
identified as being identical, e.g., by comparing the inodes of the files to detect cases where links are used to
refer the same file), the above order also identifies the precedence of loading. A file located by method a)
(previously shown in this subclause) shall override files specified by method b).
All compiled object code needs to be loaded in the specification order similarly to the preceding scheme;
first the content of the bootstrap file is processed starting with the first line, then the set of -sv_lib
switches is processed in order of their occurrence. Any library shall only be loaded once.
J.4.1 Bootstrap file
The object code bootstrap file has the following syntax:
a) The first line contains the string #!SV_LIBRARIES.
b) An arbitrary amount of entries follow, one entry per line, where every entry holds exactly one
library location. Each entry consists only of the pathname_without_extension of the object
code file to be loaded and can be surrounded by an arbitrary number of blanks; at least one blank
shall precede the entry in the line. The value pathname_without_extension is equivalent to
the value of the switch -sv_lib.
c) Any amount of comment lines can be interspersed between the entry lines; a comment line starts
with the character # after an arbitrary (including zero) amount of blanks and is terminated with a
newline character.
J.4.2 Examples
a) If the path-name root has been set by the switch -sv_root to /home/user and the following object
files need to be included:
/home/user/myclibs/lib1.so
/home/user/myclibs/lib3.so
/home/user/proj1/clibs/lib4.so
/home/user/proj3/clibs/lib2.so
then use either of the methods in FigureJ.2. Both methods are equivalent.
...
#!SV_LIBRARIES -sv_lib myclibs/lib1
myclibs/lib1 -sv_lib myclibs/lib3
myclibs/lib3 -sv_lib proj1/clibs/lib4
proj1/clibs/lib4 -sv_lib proj3/clibs/lib2
proj3/clibs/lib2 ...
Bootstrap file method Switch list method
Figure J.2—Using a simple bootstrap file or a switch list

<!-- Page 1306 -->

IEEE Std b) If the current working directory is /home/user, using the series of switches shown in FigureJ.3
(left column) results in loading the following files (right column):
-sv_lib svLibrary1 /home/user/svLibrary1.so
-sv_lib svLibrary2 /home/user/svLibrary2.so
-sv_root /home/project2/shared_code
-sv_lib svLibrary3 /home/project2/shared_code/svLibrary3.so
-sv_root /home/project3/code
-sv_lib svLibrary4 /home/project3/code/svLibrary4.so
Switches Files
Figure J.3—Using a combination of -sv_lib and -sv_root switches
c) Further, using the set of switches and contents of bootstrap files shown in FigureJ.4:
-sv_root /home/usr1 bootstrap1: #! SV_LIBRARIES
-sv_liblist bootstrap1 lib1
lib2
-sv_root /home/usr2
-sv_liblist /home/mine/bootstrap2 bootstrap2: #! SV_LIBRARIES
lib3
/common/libx
lib5
Figure J.4—Mixing -sv_root and bootstrap files
results in loading the following files:
/home/usr1/lib1.ext
/home/usr1/lib2.ext
/home/usr2/lib3.ext
/common/libx.ext
/home/usr2/lib5.ext
where ext stands for the actual extension of the corresponding file.

<!-- Page 1307 -->

IEEE Std 1800-2023