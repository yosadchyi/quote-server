# Quote server with anti DDoS

This server uses [Proof Of Work](https://en.wikipedia.org/wiki/Proof_of_work) algorithm to implement anti DDoS
security. **Solution-verification** variation of this algorithm is used, to achive:
- simplistic client-server interaction,
- minimise communication overhead (challenge-response would require additional turn around).

As a core of verification [Hashcash](https://en.wikipedia.org/wiki/Hashcash) algorithm is used, with SHA1 hash
function, which, in general can be replaced with more complicated one, like SHA3.

## How to run

To run the project just type

```make run```

it will create docker container, build project and bring up server (in background) and then client, which
will emit 10 tokens and retrieve 10 quotes in response.

## Notes

- Complexity bits can be dynamic and can be adjusted (e.g. logarithmically) with growing number of requests
- Complexity is set to 5, which is extremely low number, on production systems usual number starts from 20
- Code is keept as simple as possible, some error handling is simplified
