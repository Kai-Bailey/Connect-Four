pub fn main() {
    // ScoreBoardCtrl.js
    js! {
        angular.module("Connect4App").controller("ScoreBoardCtrl", ScoreBoardCtrl);

        angular.module("Connect4App").factory("postService", function(S_resource){
            return S_resource("/games");
        });

        function ScoreBoardCtrl(postService, S_scope, S_rootScope){
            S_scope.games = postService.query();
        };
    }
}
