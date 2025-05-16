## ccgen
### *Generate manually maintained C (and C++) headers*

ccgen is a simple rust library designed for solving the issues of keeping headers of a complex library consistent.

For example the C standard specifies that certain types must be defined in several headers
 - size_t is in stddef.h, string.h, and wchar.h
 - wchar_t is in stddef.h and wchar.h
 - (for other examples see ISO 9899:2024)

ccgen allows updates to one type to apply to each header

#### ccgen is designed for manually maintained headers for comples libraries
