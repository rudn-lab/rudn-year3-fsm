import json
import traceback
import sys

OK = 0
WRONG_ANSWER = 1
PRESENTATION_ERROR = 2
FAIL = 3

inf = open(sys.argv[1])   # testdata
ouf = open(sys.argv[2])   # contestant output
# ans not needed


def exit(code):
    raise SystemExit(code)

def word_logic(word) -> bool:
    return '000' not in word


try:
    try:
        data = ouf.read()
    except:
        print('Failed to read user input or test input')
        traceback.print_exc()
        exit(PRESENTATION_ERROR)
    try:
        word = inf.read().strip()
    except:
        print('Failed to read test input')
        traceback.print_exc()
        exit(FAIL)

    try:
        data = json.loads(data)
    except Exception as e:
        print("Output is not valid JSON: " + str(e))
        exit(PRESENTATION_ERROR)
    
    # Build the FSM
    nodes = []
    links = []
    starts = []
    try:
        for node in data['nodes']:
            nodes.append({'accept': node['isAcceptState']})
        for link in data['links']:
            if link['type'] == 'StartLink':
                n = int(link['node'])
                if n not in range(len(nodes)):
                    print(f"Start link to node {n} which does not exist")
                    exit(PRESENTATION_ERROR)
                starts.append((n, str(link['text'])))
            elif link['type'] == 'SelfLink':
                l = int(link['node'])
                if l not in range(len(nodes)):
                    print(f"Self-link at node {l} which does not exist")
                    exit(PRESENTATION_ERROR)

                links.append((l, l, str(link['text'])))

            else:
                la = int(link['nodeA'])
                lb = int(link['nodeB'])
                if la not in range(len(nodes)):
                    print(f"Link starts at node {la} which does not exist")
                    exit(PRESENTATION_ERROR)
                if lb not in range(len(nodes)):
                    print(f"Link ends at node {lb} which does not exist")
                    exit(PRESENTATION_ERROR)

                links.append((la, lb, str(link['text'])))

    except Exception:
        print("Error building FSM from data")
        traceback.print_exc()
        exit(PRESENTATION_ERROR)

    if not starts:
        print("FSM does not have any start links, so it will always reject")
        exit(PRESENTATION_ERROR)

    # Now evaluate the FSM

    # index into word = first letter we're going to test
    node_cursors = [] # (node idx, index into word)
    for node_idx, prefix in starts:
        # Starts are a special case: not like a link.
        # Check their prefix immediately.
        if word.startswith(prefix):
            node_cursors.append((node_idx, len(prefix)))
    
    while node_cursors:
        print(node_cursors)
        # Build link cursors: collect all the links leading out of the current nodes.
        link_cursors = []
        for node_idx, word_idx in node_cursors:
            # If the node cursor is pointing past the end of a word, do not consider it.
            if word_idx >= len(word): continue
            for i, link in enumerate(links):
                node_a, node_b, prefix = link
                if node_idx == node_a:
                    link_cursors.append((i, word_idx))
        
        # Clear the current node cursors, then create new ones from the links.
        node_cursors.clear()
        for link_idx, word_idx in link_cursors:
            node_a, node_b, prefix = links[link_idx]
            if word[word_idx:].startswith(prefix):
                node_cursors.append((node_b, word_idx + len(prefix)))
        
        # Check for accepts: nodes whose cursors are pointing past the word.
        # If there are any, stop and say accept.
        for node_idx, word_idx in node_cursors:
            node = nodes[node_idx]
            if word_idx == len(word):
                if node['accept']:
                    print(f'Cursor found at end of word in node {node_idx}, which ACCEPTs')
                    # Automaton accepts
                    if word_logic(word):
                        print("Automaton's answer matches")
                        exit(OK)
                    else:
                        print("Automaton's answer is wrong: should be REJECT, but it ACCEPT")
                        exit(WRONG_ANSWER)


        # If there are no node cursors left, we reject.
        if not node_cursors:
            print('All cursors fell off: REJECT')
            if not word_logic(word):
                print("Automaton's answer matches")
                exit(OK)
            else:
                print("Automaton's answer is wrong: should be ACCEPT, but it REJECT")
                exit(WRONG_ANSWER)


    exit(OK)
except Exception as e:  # only handle exceptions other than SystemExit
    # If any unhandled error occurs, exit with code 3 (FAIL), so that the jury looks at this.
    traceback.print_exc()
    print("Unexpected error: " + str(e))
    raise SystemExit(FAIL)