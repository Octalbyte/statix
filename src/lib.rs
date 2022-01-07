mod crt {

    pub fn public(_filename: String) -> Vec<u8> {
        /*error supressor */
        return vec![1, 2, 3];
    }
    pub fn private(_filename: String) -> Vec<u8> {
        /* error suppressor */
        return vec![1, 2, 3];
    }
}
