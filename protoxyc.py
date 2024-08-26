import protoxy

fds = protoxy.compile(['protos/foo.proto'], ['protos'])
with open("foo.pb", "wb") as f:
    f.write(fds.SerializeToString())