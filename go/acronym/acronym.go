package acronym

import "bytes"
import "unicode"

const testVersion = 3

func Abbreviate(input string) string {
    var buffer bytes.Buffer

    isAcronymLetter := true
    for _, c := range input {
        if unicode.IsLetter(c) && isAcronymLetter {
            buffer.WriteRune(unicode.ToUpper(c))
            isAcronymLetter = false
        } else if !unicode.IsLetter(c) {
            isAcronymLetter = true
        }
    }

    return buffer.String()
}
