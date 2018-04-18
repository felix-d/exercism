package gigasecond

// import path for the time package from the standard library
import "time"
import "math"

const testVersion = 4

func AddGigasecond(birthday time.Time) time.Time {
    return birthday.Add(time.Second * time.Duration(math.Pow(10, 9)))
}
