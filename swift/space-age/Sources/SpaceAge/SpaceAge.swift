enum PlanetOrbitalPeriod: Double {
    case Mercury = 0.2408467
    case Venus = 0.61519726
    case Mars = 1.8808158
    case Jupyter = 11.862615
    case Saturn =  29.447498
    case Uranus = 84.016846
    case Neptune = 164.79132
}

let secondsPerYearOnEarth = 31557600.0

class SpaceAge {
    let seconds: Double
    let onEarth: Double
    lazy var onMercury = onEarth / PlanetOrbitalPeriod.Mercury.rawValue
    lazy var onVenus = onEarth / PlanetOrbitalPeriod.Venus.rawValue
    lazy var onMars = onEarth / PlanetOrbitalPeriod.Mars.rawValue
    lazy var onJupiter = onEarth / PlanetOrbitalPeriod.Jupyter.rawValue
    lazy var onSaturn = onEarth / PlanetOrbitalPeriod.Saturn.rawValue
    lazy var onUranus = onEarth / PlanetOrbitalPeriod.Uranus.rawValue
    lazy var onNeptune = onEarth / PlanetOrbitalPeriod.Neptune.rawValue
    
    init(_ seconds: Int) {
        self.seconds = Double(seconds)
        onEarth = self.seconds / secondsPerYearOnEarth
    }
}
