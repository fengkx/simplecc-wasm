import {describe, test, expect} from 'vitest'
import {simplecc} from '..'

describe.concurrent('suite', () => {
    test('basic', () => {
        expect(simplecc("发财了去植发", "s2t")).toBe('發財了去植髮')
        expect(simplecc('發財了去植髮', "t2s")).toBe("发财了去植发")
    })
})