# Celestials
sol = Sol
mercury = Mercury
venus = Venus
earth = Earth
mars = Mars
jupiter = Jupiter
saturn = Saturn
uranus = Uranus
neptune = Neptune

# HUD
# in-space = In Space
near = Near { $celestial ->
        [sol] { sol }
        [mercury] { mercury }
        [venus] { venus }
        [earth] { earth }
        [mars] { mars }
        [jupiter] { jupiter }
        [saturn] { saturn }
        [uranus] { uranus }
        [neptune] { neptune }
       *[other] Anomaly
    }

# Pause
pause = Pause

# Game Over
game-over = Game Over

# Datetime
# TODO: Pass date and time as a workable datetime value for translation
# time = { DATETIME($datetime) }
