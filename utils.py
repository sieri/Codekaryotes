
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
