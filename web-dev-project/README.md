# Q&A Web

```bash
curl -X OPTIONS -v localhost:3030/questions \
    -H "Access-Control-Request-Method: PUT"\
    -H "Access-Control-Request-Headers: not-in-the-request"\
    -H "Origin: https://not-origin.io"
```
