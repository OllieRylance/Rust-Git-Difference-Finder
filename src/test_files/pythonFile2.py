def greet(name):
    return f"Hi, {name}! Welcome back."

def subtract(a, b):
    return a - b

def main():
    name = "Bob"
    print(greet(name))
    print(f"Difference: {subtract(10, 4)}")

if __name__ == "__main__":
    main()
