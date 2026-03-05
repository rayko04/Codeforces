import sys

input = sys.stdin.readline

def solve(n, f, a, b, msg_arr):
    battery = f
    prev = 0
    for m in msg_arr:
        gap = m-prev
        battery -= min(gap*a, b)
        if battery <= 0:
            return "NO"
        prev = m
    return "Yes"

def main():
    t = int(input())
    
    for _ in range(t):
        n, f, a, b = map(int, input().split())
        msg_arr = list(map(int, input().split()))
        
        print(solve(n, f, a, b, msg_arr))
        
if __name__ == "__main__":
    main()
