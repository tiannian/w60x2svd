import glob

def main():
    for path in glob.glob('gpiob/*.csv'):
        print(path)
        f = open(path, "rb")
        tf = open('target/%s' % (path), "wb+")
        for line in f:
            decodeline = line.decode("mbcs")
            tf.write(decodeline.encode('utf-8'))

main()
