class Codekaryote:

    def __init__(self, starting_position):
        """
        :param starting_position: the position the Codekaryotes spawn in
        :type starting_position: ``Position``
        """
        self._position = starting_position
    # end def __init__

    # -------------------Methods--------------------

    def update(self):
        self._position.x += 1
    # end def update

    # -----------------Properties------------------

    @property
    def position(self):
        return self._position
    # end def position
# end class Codekaryotes


class BaseModule:
    """
    A base module for systems that can evolve independently
    """
    # -------------------Methods--------------------

    # -----------------Properties------------------

# end class BaseModule