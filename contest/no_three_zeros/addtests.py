import bs4
import zipfile
import random

path = input('Path to problem package: ')

zip = zipfile.ZipFile(path)

def generate_testcase(i, do_accept=None):
    random.seed(i)
    # is this testcase ACCEPT or REJECT
    accept = do_accept or random.randint(0, 1) 
    size = random.randint(1, 15)
    if not accept: size = max(size, 3)
    o = []
    while not o:
        for _ in range(size):
            o.append(str(random.randint(0, 1)))
        
        o = ''.join(o)
        if ('000' in o) == accept:  # if we're supposed to accept, but there are 3 zeros; or if we're supposed to reject, but there aren't any
            o = []

    return o

problem = zip.open('problem.xml', 'r').read().decode()
soup = bs4.BeautifulSoup(problem)

# Clear out the problem->judging->testset[name = tests]->tests
soup.find('tests').clear()

tests_sample = []
for i in range(2):
    print(i)
    tests_sample.append(generate_testcase(-i, do_accept=True))
for i in range(2):
    print(i)
    tests_sample.append(generate_testcase(-i-2, do_accept=False))

tests = []
for i in range(80):
    print(i)
    tests.append(generate_testcase(i))

soup.find('test-count').clear()
soup.find('test-count').append(str(len(tests_sample + tests)))
test_elem = soup.find('tests')
for test in tests_sample:
    tag = soup.new_tag('test', method='manual', ) #sample='true')  TODO: samples are hard to make
    test_elem.append(tag)

for test in tests:
    tag = soup.new_tag('test', method='manual')
    test_elem.append(tag)

print('Opening for writing...')
newzip = zipfile.ZipFile(path+'.autotests.zip', mode='w')

newzip.open('problem.xml', 'w').write(str(soup).encode())

for i, v in enumerate(tests_sample + tests):
    print(f'Writing {i}')
    newzip.open(f'tests/{str(i).zfill(2)}', 'w').write(v.encode() + b'\r\n')

print('Copying rest of files over')
for file in zip.namelist():
    if file != 'problem.xml' and not file.startswith('tests/'):
        print(file)
        newzip.open(file, 'w').write(zip.read(file))
    else:
        print('SKIPPING', file)

print('Closing')
newzip.close()