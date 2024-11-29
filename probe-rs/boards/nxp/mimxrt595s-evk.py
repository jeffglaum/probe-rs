import time

def reset_flash(interface, soc_type):
    print("INFO: reset flash (soc=" + soc_type + ")");
    interface.write_word_32(0x40001044, 1 << 24); # enable GPIO clock
    interface.write_word_32(0x40000074, 1 << 24); # take GPIO out of reset
    interface.write_word_32(0x40004214, 0x130); # full drive and pullup
    interface.write_word_32(0x40102010, 1 << 5); # PIO4_5 is an output
    interface.write_word_32(0x40103214, 0); # PIO4_5 is driven low
    time.sleep(100 / 1000)
    interface.write_word_32(0x40102010, 0); # PIO4_5 is an input
    interface.flush();
    time.sleep(100 / 1000)