import math

MAX32 = (2**32)-1


def test_bit(val, index):
    """
    test if the bit at index is 1
    :param val: the value to check the bit of
    :type val: ``int``
    :param index: the index in the binary representation to check
    :type index: ``int``
    :return: flag indicating if the bit is 1
    :rtype: ``bool``
    """
    mask = 1 << index
    return (val & mask) > 0
# end def test_bit


def set_bit(val, index):
    """
    set the bit at index to 1
    :param val: the value to check the bit of
    :type val: ``int``
    :param index: the index in the binary representation to set
    :type index: ``int``
    :return: the value now with the bit set
    :rtype: ``int``
    """
    mask = 1 << index
    return val | mask
# end def set_bit


def clear_bit(val, index):
    """
    clear the bit at index to 0
    :param val: the value to check the bit of
    :type val: ``int``
    :param index: the index in the binary representation to clear
    :type index: ``int``
    :return: the value now with the bit clear
    :rtype: ``int``
    """
    mask = ~(1 << index)
    return val & mask
# end def set_bit


def toggle_bit(val, index):
    """
    toggle the bit at index
    :param val: the value to check the bit of
    :type val: ``int``
    :param index: the index in the binary representation to toggle
    :type index: ``int``
    :return: the value now with the bit toggle
    :rtype: ``int``
    """
    mask = 1 << index
    return val ^ mask
# end def set_bit


def bit_range(val, start, length):
    """
    get a range from an int
    :param val: the value to check the bit of
    :type val: ``int``
    :param start: the index in the binary range starting from the LSB
    :type start: ``int``
    :param length: the number of bits to get in
    :type length: ``int``
    :return: the value now with the bit toggle
    :rtype: ``int``
    """

    mask = (2**length-1) << start
    return (val & mask) >> start
# end def bit_range

def to_signed(val, n_bit):
    """
    transform a number to a signed integer
    :param val: the value to convert
    :type val: ``int``
    :param n_bit: number of bit representing that number
    :type n_bit: ``int``
    :return: the signed number
    :rtype: ``int``
    """
    if test_bit(val, n_bit-1):
        return -bit_range(val, 0, n_bit-1)
    else:
        return val


def clamp(n, smallest, largest):
    """
    clamp between two values
    :param n: the number to clamp
    :type n: ``float``
    :param smallest: the smaller number
    :type smallest: ``float``
    :param largest: the bigger number
    :type largest: ``float``
    :return: the number clamped
    :rtype: ``float``
    """
    return max(smallest, min(n, largest))
# end def clamp

def scale_between(n, smallest, largest, initial_smallest=0, initial_largest=MAX32):
    """
    clamp between two values, by scaling between them the input
    :param n: the number to clamp
    :type n: ``float`` or ``int``
    :param smallest: the smaller number
    :type smallest: ``float`` or ``int``
    :param largest: the bigger number
    :type largest: ``float`` or ``int``
    :return: the number clamped
    :rtype: ``float``
    """
    factor = float(initial_largest-initial_smallest)/float(largest-smallest)
    return (n-initial_smallest)/factor + smallest
# end def clamp
def dist(organism, pos):
    pos2 = organism.position
    return math.sqrt((pos2.x-pos.x)**2+(pos2.y-pos.y)**2)


def angle(organism, pos):
    pos2 = organism.position
    return math.atan2(pos2.y-pos.y, pos2.x-pos.x)
