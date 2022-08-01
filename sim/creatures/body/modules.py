from sim.creatures.codekaryote import BaseModule


class Movement(BaseModule):

    def __init__(self, creature, genome):
        super().__init__(creature, genome, "movement")
        creature.__setattr__("movement_module", self)
    # end def __init__

    # -------------------Methods--------------------

    def update(self):
        pass
    # end def update

    def move_up(self):
        self._creature.position.y += 1
    # end def move_up

    def move_down(self):
        self._creature.position.y -= 1
    # end def move_down

    def move_right(self):
        self._creature.position.x += 1
    # end def move_right

    def move_left(self):
        self._creature.position.x -= 1
    # end def move_left



    # -----------------Properties------------------


# end class Mouvement