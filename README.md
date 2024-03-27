```
$ cargo run --bin mybin1
1.25.1
ok
```

```
$ cargo run --bin mybin2
error: ImportError:

IMPORTANT: PLEASE READ THIS FOR ADVICE ON HOW TO SOLVE THIS ISSUE!

Importing the numpy C-extensions failed. This error can happen for
many reasons, often due to issues with your setup or how NumPy was
installed.

We have compiled some common reasons and troubleshooting tips at:

    https://numpy.org/devdocs/user/troubleshooting-importerror.html

Please note and check the following:

  * The Python version is: Python3.10 from "/usr/bin/python3"
  * The NumPy version is: "1.25.1"

and make sure that they are the versions you expect.
Please carefully study the documentation linked above for further help.

Original error was: /home/user/.local/lib/python3.10/site-packages/numpy/core/_multiarray_umath.cpython-310-x86_64-linux-gnu.so: undefined symbol: PyObject_SelfIter
```
