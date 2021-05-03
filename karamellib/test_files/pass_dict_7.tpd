data = { 'key_1': 'evet' }
data.key_1 = 'hayır'
hataayıklama::doğrula(data.key_1, 'hayır')
hataayıklama::doğrula(data['key_1'], 'hayır')
hataayıklama::doğrula(data.getir('key_1'), 'hayır')
hataayıklama::doğrula(data.getir('key_2'), boş)

data.güncelle('key_1', 'evet')
hataayıklama::doğrula(data.getir('key_1'), 'evet')

data.güncelle('key_2', 'erhan')
hataayıklama::doğrula(data.getir('key_2'), 'erhan')
hataayıklama::doğrula(data.uzunluk(), 2)
hataayıklama::doğrula(data.anahtarlar() == ['key_1', 'key_2'] veya data.anahtarlar() == ['key_2', 'key_1'] , doğru)

hataayıklama::doğrula(data.içeriyormu("key_1"), doğru)
hataayıklama::doğrula(data.içeriyormu("key"), yanlış)

data.temizle()
hataayıklama::doğrula(data.uzunluk(), 0)


fonk a(): döndür 10
data.key_2 = a
hataayıklama::doğrula(data.getir('key_2')(), 10)    