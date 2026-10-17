## LCOV

Link to its [repo](https://github.com/linux-test-project/lcov).

### What is it?

According to the lcov docs:

> LCOV is a tool to manipulate and display information about what parts of a program are actually executed (i.e. "covered") while running a particular test case or set of testcases

Essentially, it is a representation of code test coverage in a simple line-by-line record format. It also provides some other helpful tools to manipulate it, such as `genhtml`.

### Records

| Record        | Name                   | Format            |
| ------------- | ---------------------- | ----------------- |
| TN            | Test name              | TN:app_test.dart  |
| SF            | Source file            | SF:/lib/file.dart |
| FN            | Function name          | FN:44,func1       |
| FNDA          | Function execute count | FNDA:10,func1     |
| FNF           | Function count         | FNF:23            |
| FNH           | Functions hit          | FNH:14            |
| DA            | Line execute count     | DA:23,4           |
| LF            | Line count             | LF:123            |
| LH            | Lines hit              | LH:97             |
| BRDA          | Branch data            | BRDA:23,1,2,4     |
| LH            | Branch count           | BRF:10            |
| LH            | Branches hit           | BRH:5             |
| end_of_record | End of record          | end_of_record     |

### Entry

Each entry (or group of records) in an lcov file starts with a `TN` followed by an `SF`, with the rest in-between, and finally ending with an `end_of_record`.

### Example valid lcov

```plaintext
TN:integration_tests
SF:/home/user/project/main.cpp
FN:22,func1
FNDA:3,func1
FNF:1
FNH:1
DA:1,2
DA:2,0
DA:3,1
LF:3
LH:2
end_of_record
TN:integration_tests
SF:/home/user/project/other.cpp
DA:1,2
DA:2,0
DA:3,1
BRDA:28,0,0,3
BRDA:28,0,1,2
BRDA:29,1,0,2
BRDA:29,1,1,0
BRDA:43,2,0,-
BRDA:43,2,1,-
BRF:6
BRH:4
LF:3
LH:2
end_of_record
```
