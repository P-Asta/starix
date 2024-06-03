# starix
the github-blog bot(use discord webhook)

## post-conv
```fish
upload(name): description
# or
post(name): description
```

## fix
```fish
fix(name): description
```
## Starix.toml
```toml
# ur blog url
url="https://blog.5-23.dev/p/{starix.id}/"
# ur blog thumbnail url
thumb="https://blog.5-23.dev/p/{starix.id}/thumb.jpg"
# webhook bot name
name="Asta blog"

[post]
content="@everyone `{starix.id}`가 올라왔어요!"
color="#00ff00"

[fix]
content=""
color="#ffff00"
```