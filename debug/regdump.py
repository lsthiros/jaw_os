# Import gdb module
import gdb
import re

# Define a new gdb command called regdump
# Regdump will print the content of the GIC registers. Options are:
# GIC.ISENABLERn, GIC.ICENABLERn, GIC.ISPENDRn, GIC.ICPENDRn, GIC.ISACTIVERn, GIC.ICACTIVERn
# "n" can be any valid register number. When the register is printed, the bitfields will be
# decoded and printed as well.
class RegDump(gdb.Command):
    # GICD offset
    GICD_OFFSET = 0x08000000
    GICC_OFFSET = 0x08010000
    OFFSETS_BY_NAME = {
        "ISENABLER": 0x100,
        "ICENABLER": 0x180,
        "ICPENDR": 0x280,
        "ICFGR": 0xC00,
    }


    def __init__(self):
        super(RegDump, self).__init__("regdump", gdb.COMMAND_USER)

    def invoke(self, arg, from_tty):
        # get the register name from the argument
        reg = arg
        # Reg will be in the form of GIC.ISENBLERn. Use a regex to extract the register name after "GIC."
        # and the integer "n". Store the register name in reg_name and the integer in reg_num
        m = re.match(r'GIC\.([A-Z]+)([0-9]+)', reg)
        if m:
            reg_name = m.group(1)
            reg_num = int(m.group(2))
        else:
            print("Invalid register name")
            return
