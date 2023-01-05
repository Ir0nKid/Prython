import dis

def parse(f):
    
    print(" ========= python byte code ======")
    print()
    print(dis.dis(f))
    
    print(" ========= byte code =============")
    code = f.__code__.co_code
    print(code)
    print()
    
    with open("py/out.b","wb+") as w:
        w.write(code)


@parse
def test():
    return 1 + 1
