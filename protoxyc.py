import protoxy

fds = protoxy.compile(['protos/foo.proto'], ['protos'], include_imports=False)
with open("foo.pb", "wb") as f:
    f.write(fds.SerializeToString())