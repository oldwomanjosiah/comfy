# *comfy* command script manager
*comfy* is a command script manager / runner tool, which allows your to run commands in the command line itself, but being these predefined in a portable and universal *.comfy* file.  
*.comfy* files are plain text, but with some arguments so *comfy parser* understands what do you want to do.

```
// this is a comment
> linux
echo linux system!
> windows
echo windows system!
> always
echo this always runs!
@ sleep 2000
echo you waited 2000 ms!
```
As you are thinking, the above code only runs the commands, that should be runned, depending on the environment in which the script is runned. Also, keep in mind that everything after *always* clause will run on any operating system. *comfy* automatically detects the system, and, being programmed in Rust, allows the specification of the following systems:
- linux
- macos
- freebsd
- solaris
- android
- windows
- others

*comfy* also has some *universal functions*, they work on any system regardless of the installed libraries. *comfy* comes with several packaged libraries, so far, we have the following *universal functions*:
- @ sleep [int] (ms)

Not to mention that you can intertwine different system clauses, like this:
```
> linux
echo Linux user here!
> windows
echo Windows user here!
> linux
echo Linux user here! x2
> windows
echo Windows user here! x2
```

If you are on Linux, output should be:
```
Linux user here!
Linux user here! x2
```

*comfy* usage:  
| Command                | Use                                                    |
|------------------------|--------------------------------------------------------|
| --help                 | shows this help page and exits                         |
| --helpf                | shows help about *comfy* functions                     |
| --file <foo.comfy>     | runs *comfy* script contents                           |
| --file <foo.comfy> --c | runs *comfy* script contents, also prints comments     |