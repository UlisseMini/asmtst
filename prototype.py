import time
import struct

DEBUG = True

class Instruction():
    def __init__(self, opcode, args, name):
        self.opcode = opcode
        self.args = args
        self.name = name

    def __repr__(self):
        return "<Instruction opcode = {} args = {}>".format(self.opcode, self.args)

    def __str__(self):
        return self.name

MOV   = Instruction(1, 2, 'MOV')   # move a value to a register
STATE = Instruction(2, 0, 'STATE') # print the state of the virtual machine
ADD   = Instruction(3, 3, 'ADD')   # add two registers, store result in a third

ops = [MOV, STATE, ADD]

def interp(bytecode):
    reg = {
        'pc': 0, # program counter

        # general registers
        'g1': 0,
        'g2': 0,
        'g3': 0,
        'g4': 0,
    }
    while reg['pc'] < len(bytecode):
        op = bytecode[reg['pc']]
        reg['pc'] += 1

        if DEBUG:
            time.sleep(0.5)

        if op[0] == MOV:
            val, reg_idx = op[1], op[2]
            reg[list(reg.keys())[reg_idx]] = val
        elif op[0] == ADD:
            r1, r2, out = op[1], op[2], op[3]
            r1 = reg[list(reg.keys())[r1]]
            r2 = reg[list(reg.keys())[r2]]

            reg[list(reg.keys())[out]] = r1 + r2

        elif op[0] == STATE:
            print('==== Registers ====')
            for k in (reg):
                print('{}: {}'.format(k, reg[k]))


def b_sum(list_of_bytes):
    buf = b''
    for b in list_of_bytes:
        buf += b
    return buf

def comp(bytecode):
    actual_bytecode = b''

    i = 0
    while i < len(bytecode):
        byte = bytecode[i]
        op = byte[0]

        actual_bytecode += chr(op.opcode).encode()
        # actual_bytecode += chr(bytecode[i].args).encode()

        for j in range(1, op.args + 1):
            actual_bytecode += chr(byte[j]).encode()

        i += 1

    return actual_bytecode

def get_op_for(opcode):
    for known_op in ops:
        if known_op.opcode == opcode:
            return known_op

    raise ValueError("Unknown opcode '{}'".format(opcode))

def decomp(actual_bytecode):
    i = 0
    bytecode = []
    while i < len(actual_bytecode):
        opcode = actual_bytecode[i]
        op = get_op_for(opcode)
        args = actual_bytecode[i+1 : i + op.args + 1]
        bytecode.append((op, *args))

        i += 1 + op.args
    return bytecode

src = [
    # Initilization
    (MOV, 1, 2), # MOV 1 to the second general register

    # loops forever
    (STATE,),       # print VM state
    (ADD, 2, 1, 1), # add general register 2 and general register 1, store result in 1
    (MOV, 1, 0),    # move 1 to pc
]

# actual_bytecode = comp(src)
# with open('compiled.uo', 'wb') as f:
#     f.write(actual_bytecode)


decompiled = decomp(open('compiled.uo', 'rb').read())
interp(decompiled)
