import math
from decimal import *
import re
class bigfloat:
    def __init__(self,sign,front,back,prec):
        
        self.sign = sign
        self.front = front
        self.back = back
        self.prec = prec
    
    def __str__(self):
        return ("-" if self.sign == -1 else "")+("0" if not len(self.front) or self.front == [0]*self.prec else re.sub("^[0+]*","","".join(str(x) for x in self.front)))+(".0" if not len(self.back) or self.back == [0]*self.prec else "."+re.sub("0+(?![^1-9]*[1-9])","","".join(str(x) for x in self.back)))
    
    def fix(self,other):
        if self.prec > other.prec:
            other.front= [0]*(self.prec-other.prec) + self.front
            other.back+=[0]*(self.prec-other.prec)
            other.prec=self.prec
        elif other.prec>self.prec:
            self.front=[0]*(other.prec-self.prec)+self.front
            self.back+= [0]*(other.prec-self.prec)
            self.prec=other.prec
    def false_div(a,x):
        return parse("0."+"0"*(x-1)+str(a))
    def __gt__(self,other):
        self.fix(other)
        if self.sign != other.sign:
            return self.sign > other.sign
        else:
            for i in range(self.prec):
                if self.front[i] > other.front[i]:
                    return True
                elif other.front[i]>self.front[i]:
                    return False
            for i in range(self.prec):
                if self.back[i] > other.back[i]:
                    return True
                elif other.back[i] > self.back[i]:
                    return False
            return False
    def __lt__(self,other):
        return not self > other
    def __add__(self,other):
        self.fix(other)
        carry = 0
        rb = []
        rf = []
        if(self.sign == other.sign):
            
            for i in list(reversed(range(len(self.back)))):
                newval = self.back[i]+other.back[i]
                carry = newval//10
                newval = newval-carry*10
                if i-1 >=0:
                    self.back[i-1]+=carry
                elif carry>0:
                    self.front[-1]+=carry
                rb.append(newval)
            if carry > 0:
                rf.append(carry)
            rb.reverse()
            
            for i in list(reversed(range(len(self.front)))):
                newval = self.front[i]+other.front[i]
                carry = newval//10
                newval = newval%10
                
                if i-1 >=0 and carry >0:
                    self.front[i-1]+=carry
                elif carry>0:
                    self.front.insert(0,carry)
                rf.append(newval)
            if carry > 0:
                rf.append(carry)
            
            rf.reverse()
        return bigfloat(self.sign,rf,rb,self.prec)
    def tosdigit(a):
        return a.sign*a.front[-1] #only for -9..9
    def pow10(self,pow):
        olen = len(self.back)
        newback = self.back + [0]*pow
        extra = newback[0:len(newback)-olen]
        final = newback[len(newback)-olen:-1]
        self.front += extra
        self.back = final
        return self

    def size(self):
        return len([x for x in self.front if x !=0])+len([x for x in self.back if x !=0])
    def smul(self,a):
        rb = []
        carry = 0//10
        for i in list(reversed(range(len(self.back)))):
            newback = self.back[i]*a+carry
            carry = newback//10
            newback = newback%10
            rb.append(newback)
        rf = []
        for i in list(reversed(range(len(self.front)))):
            newfront = self.front[i]*a+carry
            carry = newfront//10
            newfront = newfront%10
            rf.append(newfront)
        if carry >0:
            rf.append(carry)
        rb.reverse()
        rf.reverse()
        return bigfloat(-1 if (a<0 or self.sign == -1) and not (self.sign==-1 and a<0) else 0,rf,rb,max(len(rf),len(rb)))
    def __sub__(self,other):
        self.fix(other)
        if self.sign == other.sign and self.sign == 0:
            if self>other:
                rb = []
                for i in list(reversed(range(len(self.back)))):
                    newval = self.back[i]-other.back[i]
                    borrow = newval//10
                    newval =  newval-borrow*10
                    if i-1 >=0:
                        self.back[i-1] +=borrow
                    else:
                        self.front[-1] +=borrow
                    rb.append(newval)
                rf = []
                for i in list(reversed(range(len(self.front)))):
                    newval = self.front[i] - other.front[i]
                    borrow = newval//10
                    newval = newval-borrow*10
                    if i-1 >=0:
                        self.front[i-1]+= borrow
                    rf.append(newval)
                rb.reverse()
                rf.reverse()
                return bigfloat(0,rf,rb,max(len(rf),len(rb)))
            elif self<other:
                new = other-self
                return bigfloat(-1,new.front,new.back,new.prec)
            else:
                return bigfloat(0,[0],[0],1)
        elif self.sign == other.sign and self.sign == -1:
            new = other + self
            return bigfloat(-1,new.front,new.back,new.prec)
    def splitat(self,n):
        comb = self.front+self.back
        return bigfloat(0,comb[:n],[0],len(comb[:n])),bigfloat(0,comb[n:],[0],len(comb[n:]))
    
    def mul(self,other,shift,sign=0):
        if self<bigfloat(0,[1,0],[0,0],2):
            return other.smul(self.tosdigit())
        elif  other<bigfloat(0,[1,0],[0,0],2):
            return self.smul(other.tosdigit())
        m = max(self.size(),other.size())
        m2 = int(math.ceil(m/2.0))
        m = m if m % 2 == 0 else m + 1
        a,b = self.splitat(m2)
        c,d = self.splitat(m2)
        ac = a.mul(c,len([x for x in a if x!=0])+len([x for x in c if x!=0]))
        bd = b.mul(d,len([x for x in d if x!=0])+len([x for x in b if x!=0]))
        


def parse(string):
        sign = 0
        if(string[0] == "-"):
            sign = -1
        else:
            sign = 0
        treated = string.replace("-","").replace("+","")
        front = []
        back = []
        prec = 0
        if len(re.findall("\.",treated)) == 1:
            front = [int(x) for x in list(treated.split(".")[0])]
            back = [int(x) for x in list(treated.split(".")[1])]
            prec = max(len(front),len(back))
            front= [0]*(prec-len(front)) + front
            back+= ([0]*(prec-len(back)))
            
        elif len(re.findall("\.",treated)) == 0:
            front = [int(x) for x in list(treated)]
            prec = len(front)
            back = [0]*prec
        front = [x for x in front if x != []]
        back = [x for x in back if x !=[]]
        return bigfloat(sign,front,back,prec)
print(parse("109876543210000").smul(9))
class Large:
    def __init__(self,sign,value,m=1):
        self.sign = sign
        self.value= value #[a,b,c,d]
        self.m = m
    def __str__(self):
        return ("-" if self.sign == -1 else "") + (str(self.m)+"*" if self.m != 1 else "") + (("(10^^^)^"+str(self.value[0])) if self.value[0] != 0 else "") + (("(10^^)^"+str(self.value[1])) if (self.value[1] != 0) else "") + (("(10^)^"+str(self.value[2])) if (self.value[2] != 0) else "") +(" |" if not not_exp(self) else "")+ str(self.value[3])
def not_exp(a):
    return a.value[0] ==0 and  a.value[1] ==0 and a.value[2] ==0
def below_tetration(a):
    return a.value[0] == 0 and a.value[1] == 0
def above_tetration(a):
    return a.value[0] > 0 or a.value[1] > 0
def gt(a,b):
    if a.sign > b.sign:
        return True
    elif a.sign < b.sign:
        return False
    if a.value[0]>b.value[0] or a.value[1]>b.value[1]:
        return True
    if a.value[0] == b.value[0]:
        if a.value[1] > b.value[1]:
            return True
        if a.value[1] == b.value[1]:
            if a.value[2] == b.value[2]:
                return a.value[3] >b.value[3]
            if a.value[2] > b.value[2]:
                return True
    return False
def lt(a,b):
    return not gt(a,b)
def false_div(a,x):
    return "0."+"0"*(x-1)+str(a)
def add(a,b):
    if below_tetration(a) and below_tetration(b):
        if a.sign == b.sign:
            if a.value[0] == b.value[0] and a.value[1] == b.value[1] and a.value[2] == b.value[2]:
                k = a.value
                if not not_exp(a) and  not not_exp(b):
                    m = 0
                    if(a.value[3]>b.value[3] and a.value[3] - b.value[3] < 50):
                        m = a.m+float(false_div(b.m,a.value[3]-b.value[3]))
                    elif( a.value[3] - b.value[3] > 50):
                        m = a.m
                    else:
                        m = a.m + b.m
                    return Large (a.sign, k, m)
                else:
                    k = a.value
                    k[3] = a.value[3]+b.value[3]
                    return Large(a.sign,k)
            else:
                if(a.value[2] == 0 and b.value[2] == 0):
                    return Large(0,[0,0,0,a.value[3]+b.value[3]])
        else:
            if(not_exp(a) and not_exp(b)):
                if(a.sign == 0):
                    #b is negative
                    s = 0 if gt(a,b) else -1
    else:
        return a if gt(a,b) else b

a = Large(0,[0,0,0,10])
b = Large(0,[0,0,0,1000])
g = Large(0,[0,0,1,20])
h = Large(0,[0,0,1,10])
print(add(a,b))
print(add(g,h))
#Sum of negative numbers
e = Large(-1,[0,0,0,10])
f = Large(-1,[0,0,0,100])
print(add(e,f))

#Greater than test
print(gt(a,b)) #Expected False
print(gt(b,a)) #Expected True
print(gt(a,e)) #Expected True

#Print test
c = Large(0,[0,0,0,10])
print(c)
