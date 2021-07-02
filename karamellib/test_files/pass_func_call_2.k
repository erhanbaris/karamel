fonk test:
    fonk test_erhan:
        döndür 'erhan'
    döndür test_erhan

data      = { }
data.func = test

hataayıklama::doğrula(data.func()(), 'erhan')