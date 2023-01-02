# DevSecOps and Site Reliability Engineering Dojo 2023-01

This is my solution to [the exercise I built this month](https://github.com/thecjharries/dso-dojo-slow-server).

## Usage

This assumes you've got all the containers defined [in the exercise](https://github.com/thecjharries/dso-dojo-slow-server) locally built.

```bash
make solution
curl 10.0.0.1/api/10
# this is slow
curl 10.0.0.1/api/10
# this is fast
make solution
```
