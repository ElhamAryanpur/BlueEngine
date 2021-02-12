local x = os.clock()

-- test 1
i = 0
arr = {}
for i=0, 1000000 do
  arr[i] = "new str"
end

print(string.format("test 1 done in: %.2f miliseconds", (os.clock() - x) * 1000) )

-- test 1
x2 = os.clock()
d = 0
for o=0, 1000000000 do
  d = d + o
end

print(string.format("test 2 done in: %.2f miliseconds", (os.clock() - x2) * 1000) )
