
class Settings:

    brain_rust = True

    def __new__(cls, *args, **kwargs):
        if not hasattr(cls, 'instance'):
            cls.instance = super().__new__(cls)
        return cls.instance

    def set_brain_rust(self, bool):
        self.brain_rust = bool