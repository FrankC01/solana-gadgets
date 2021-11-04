from borsh_construct import *
from construct.core import FormatField

x = ['name', 'age']
y = [String, U8]
slist = []
for i in range(2):
    z = (x[i] / y[i])
    slist.append(z)
# z = [(x[i] / y[i]) for i in range(2)]
r = CStruct(*slist)
print(f"{r}")
