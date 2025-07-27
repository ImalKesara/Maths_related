print("Welcome to number convertor")
print("Choose 1 or 2 for convert")
while  True:

    user_in = int(input("enter 1 or 2"))
    if user_in == 1:
        
        user_data = int(input("Enter number"))
        arr = []
        while user_data != 0:
            binary = user_data % 2
            arr.append(binary)
            user_data //= 2
        arr.reverse()
        print(arr)
        break


