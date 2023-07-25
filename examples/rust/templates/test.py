#!/usr/bin/env python3

import json

import singlestoredb as s2

# Unfortunately, the XPath library used does not support extracting text using `text()`, so we had to work
# around it by adding a text attribute that gets looked up after querying the XPath.
JSON_TEMPLATE = r'''{
    "book": {
        "title": {{ q(path="/book/title") | get(key="text") | json_encode | safe }},
        "authors": [{% for item in q(path="/book/author") %}
                        {{ item | get(key="text") | json_encode | safe }}{% if not loop.last %}, {% endif %}
                    {% endfor %}],
        "published": { "year": {{ q(path="/book/year") | get(key="text") | int }} },
        "listing": { "price": {{ q(path="/book/price") | get(key="text") | float }} }
    }
}'''

# Templates that use a JSON object can use either the object notation in the templating engine with
# `_` as the top element, or you can use the `q(...)` function to query using JSONPath queries.
XML_TEMPLATE = r'''<book>
    <title>{{ q(path="$.book.title") | escape_xml | safe }}</title>
    {% for author in _.book.authors %}<author>{{ author | escape_xml | safe }}</author>{% if not loop.last %}\n    {% endif %}{% endfor %}
    <year>{{ _.book.published.year | int }}</year>
    <price>{{ _.book.listing.price | float }}</price>
</book>
'''

# This template generates yaml from a JSON object
YAML_TEMPLATE = r'''book:
    title: {{ _.book.title }}
    authors:
        {% for author in _.book.authors %}- {{ author }}{% if not loop.last %}\n        {% endif %}{% endfor %}
    published:
        year: {{ _.book.published.year | int }}
    listing:
        price: {{ _.book.listing.price | float }}
'''

with s2.connect('root:@localhost:9306') as conn:
    with conn.cursor() as cur:
        cmds = [
            '''CREATE DATABASE IF NOT EXISTS xml_test''',
            '''USE xml_test''',
            '''DROP TABLE IF EXISTS books''',
            '''CREATE TABLE books (
                   id INT,
                   xml TEXT
            )''',
            f"""SET @json_template = '{JSON_TEMPLATE}'""",
            f"""SET @xml_template = '{XML_TEMPLATE}'""",
            f"""SET @yaml_template = '{YAML_TEMPLATE}'""",
        ]
        for cmd in cmds:
            cur.execute(cmd)

        cur.executemany(
            r'''INSERT INTO books (id, xml) VALUES (%s, %s)''', 
            [
                (1, '''<book category="web">\n'''
                    '''    <title lang="en">XQuery Kick Start</title>\n'''
                    '''    <author>James McGovern</author>\n'''
                    '''    <author>Per Bothner</author>\n'''
                    '''    <author>Kurt Cagle</author>\n'''
                    '''    <author>James Linn</author>\n'''
                    '''    <author>Vaidyanathan Nagarajan</author>\n'''
                    '''    <year>2003</year>\n'''
                    '''    <price>49.99</price>\n'''
                    '''</book>\n'''), 
            ],
        )
        cur.execute(
            '''SELECT xml, '''
            '''       render_xml(xml, @json_template), '''
            '''       render_json(render_xml(xml, @json_template), @xml_template), '''
            '''       render_json(render_xml(xml, @json_template), @yaml_template), '''
            '''       render_yaml(render_json(render_xml(xml, @json_template), @yaml_template), @xml_template) '''
            '''FROM books''',
        )
        for row in cur:
            print('# XML input')
            print(row[0])

            print('# JSON output')
            print(json.dumps(json.loads(row[1]), indent=2), end='\n\n')

            print('# XML output')
            print(row[2])

            print('# YAML output')
            print(row[3])

            print('# XML => JSON => YAML => XML')
            print(row[4])