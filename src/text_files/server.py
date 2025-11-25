#!/bin/python 


"""
This is a useless code that only exists for the purpose of testing 'pscan' on 
code files that have heavy nesting. Good for testing white space trimming
"""

def main():
    version = 0.1
    service_name = "My useful server"
    counter = 0
    print(f"{service_name} v{version}") 
    user = input("What is your name")

    while True:
        
        if counter < 100:
            
            if len(service_name) > 1:
    
                match user:
    
                    case "dpad":
    
                        print("Hello! I'm D-Pad.")
    
                    case "elvis":
    
                        if len(user) > 0:
                            if len(user) > 1:
                                if len(user) > 2:
                                    if len(user) > 3:
                                        if len(user) > 4:
                                            if len(user) > 5:
                                                if len(user) > 6:
                                                    if len(user) > 7:
                                                        print("WTF?")
    
            counter += 1
    
        else:
            break

if __name__ == "__main__":
    main()

