import time

def reset_flash(interface, soc_type):
    print("INFO: reset flash (soc=" + soc_type + ")");
    interface.write_word_32(0x40021044, 1 << 2); # enable HSGPIO2 clock
    interface.write_word_32(0x40000074, 1 << 2); # take HSGPIO2 out of reset
    interface.write_word_32(0x40004130, 0x130);  # full drive and pullup
    interface.write_word_32(0x40102008, 1 << 12); # PIO2_12 is an output
    interface.write_word_32(0x40102288, 1 << 12); # PIO2_12 is driven low
    time.sleep(100 / 1000)
    interface.write_word_32(0x40102208, 1 << 12); # PIO2_12 is driven high
    interface.flush();
    time.sleep(100 / 1000)