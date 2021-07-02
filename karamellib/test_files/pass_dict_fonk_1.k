fonk test(değer): döndür değer * 2 

data = {}
data.f = test

hataayıklama::doğrula(data.f(2), 4)
hataayıklama::doğrula(data['f'](2), 4)

hataayıklama::doğrula(data.f(10), 20)
hataayıklama::doğrula(data['f'](10), 20)