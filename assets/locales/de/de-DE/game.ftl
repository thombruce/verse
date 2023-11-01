# Celestials
sol = Sol
mercury = Merkur
venus = Venus
earth = Erde
mars = Mars
jupiter = Jupiter
saturn = Saturn
uranus = Uranus
neptune = Neptun

# HUD
# in-space = In Space
near = In der Nähe von { $celestial ->
        [sol] { sol }
        [mercury] { mercury }
        [venus] { venus }
        [earth] { earth }
        [mars] { mars }
        [jupiter] { jupiter }
        [saturn] { saturn }
        [uranus] { uranus }
        [neptune] { neptune }
       *[other] Anomalie
    }
# TODO: Genders (von vs der), and Anomalie should probably be "einer anomalie"

# Pause
pause = Pause

# Datetime
# TODO: Pass date and time as a workable datetime value for translation
# time = { DATETIME($datetime) }
