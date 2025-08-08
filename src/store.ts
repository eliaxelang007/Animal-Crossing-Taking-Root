// https://github.com/eliaxelang007/Handy/blob/master/lib/src/cache.dart
class Store<I, O> {
    private readonly cache: Map<I, O>;

    constructor(
        private readonly generator: (input: I) => O
    ) {
        this.cache = new Map();
    }

    get(input: I): O {
        const cached_output = this.cache.get(input);

        if (cached_output !== undefined) return cached_output;

        const result = this.generator(input);
        this.cache.set(input, result);

        return result;
    }
}

export { Store };