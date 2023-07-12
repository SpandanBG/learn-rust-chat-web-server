## Http Get Request

#### Request Structure

The HTTP Get request received by the server is of the following format

```
"GET /styles.css HTTP/1.1",
"Host: localhost:8080",
"Connection: keep-alive",
"sec-ch-ua: \"Not.A/Brand\";v=\"8\", \"Chromium\";v=\"114\", \"Google Chrome\";v=\"114\"",
"DNT: 1",
"sec-ch-ua-mobile: ?0",
"User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36",
"sec-ch-ua-platform: \"Windows\"",
"Accept: text/css,*/*;q=0.1",
"Sec-Fetch-Site: same-origin",
"Sec-Fetch-Mode: no-cors",
"Sec-Fetch-Dest: style",
"Referer: http://localhost:8080/",
"Accept-Encoding: gzip, deflate, br",
"Accept-Language: en-US,en;q=0.9",
```

#### Get Request Handling

We need to break the response into the following parts:
1. The `Get /styles.css HTTP/1.1` is the path request.
2. The remainder are the headers.
