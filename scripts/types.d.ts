declare module '*.csv' {
    const val: {
        [index: string | number]: string
    }[];
    export default val;
}