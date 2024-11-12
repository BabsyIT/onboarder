# Nach Login kannst du immer noch die User Info anfordern mit:
curl --location --request POST 'https://babsy.xtrem.ch/me' \
--header 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJodHRwczovL2JhYnN5Lnh0cmVtLmNoL2xvZ2luIiwiaWF0IjoxNzMxNDA1NzM1LCJleHAiOjE3NjI5NDE3MzUsIm5iZiI6MTczMTQwNTczNSwianRpIjoibHVabE1ZTGNBSDkzY1JnUyIsInN1YiI6IjUiLCJwcnYiOiIyM2JkNWM4OTQ5ZjYwMGFkYjM5ZTcwMWM0MDA4NzJkYjdhNTk3NmY3In0.T4ZkrCtAL7z6jQDbTWieMKNXhIyX2RYoihgyOk9iogE' \
--data ''
# Damit wir nach den Superbabsys und Admins zu filtern und den anderen das Login zu verweigern
