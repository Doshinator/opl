#include "test_infra.h"
#include <print>

static int tests_passed = 0;
static int tests_failed = 0;

void TEST_EXPRESSION(
    const std::string &name, 
    int actual, 
    int expected
) {
    if (actual == expected) {
        std::println("[PASS] {}", name);
        tests_passed++;
    } else {
        std::println("[FAIL] {}", name);
        std::println("       expected: {}", expected);
        std::println("       actual:   {}", actual);
        tests_failed++;
    }
}

void TEST_SUMMARY() {
    std::println(
        "\nSummary: {} passed, {} failed",
        tests_passed,
        tests_failed
    );
}