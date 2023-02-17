# cdfy
Card game plugin system using WebAssembly.

# develop
```
docker-compose up -d
just serve
cd client && npm run dev
```

# deploy
- server(Deno deploy via `deployctl`)
- redis(Upstash)
- client(Vercel)

```
just release
```

# TBD
- custom plugin web ui view (how?)